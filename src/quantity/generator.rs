use std::fs::File;
use std::io::Result;
use std::io::Write;

const MINIMUM: i32 = -10;
const MAXIMUM: i32 = 10;

pub fn generate_scale_mul() -> Result<()> {
    let mut file = File::create("src/quantity/scale_mul.rs")?;
    file.write_all(b"use super::*;\n\n")?;

    for i in MINIMUM..=MAXIMUM {
        file.write_all(
            format!(
                "\n//------------------------- {} -------------------------\n",
                i
            )
            .as_bytes(),
        )?;

        if i == 0 {
            file.write_all(
                r#"
impl<P1, P2, const N: i32> MulScale<Scale<P2, N>> for Scale<P1, 0>
where
    P1: Prefix,
    P2: Prefix,
{
type Output = Scale<P2, N>;
}
"#
                .as_bytes(),
            )?;
        } else {
            for j in MINIMUM..=MAXIMUM {
                let res = i + j;
                if res >= MINIMUM && res <= MAXIMUM {
                    if j == 0 {
                        file.write_all(
                            format!(
                                r#"
impl<P1, P2> MulScale<Scale<P2, 0>> for Scale<P1, {}>
where
    P1: Prefix,
    P2: Prefix,
{{
    type Output = Scale<P1, {}>;
}}
"#,
                                i, res
                            )
                            .as_bytes(),
                        )?;
                    } else if res == 0 {
                        file.write_all(
                            format!(
                                r#"
impl<P> MulScale<Scale<P, {}>> for Scale<P, {}>
where
    P: Prefix,
{{
    type Output = Scale<One, 0>;
}}
"#,
                                j, i,
                            )
                            .as_bytes(),
                        )?;
                    } else {
                        file.write_all(
                            format!(
                                r#"
impl<P> MulScale<Scale<P, {}>> for Scale<P, {}>
where
    P: Prefix,
{{
    type Output = Scale<P, {}>;
}}
"#,
                                j, i, res
                            )
                            .as_bytes(),
                        )?;
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn generate_scale_div() -> Result<()> {
    let mut file = File::create("src/quantity/scale_div.rs")?;
    file.write_all(b"use super::*;\n\n")?;

    for i in MINIMUM..=MAXIMUM {
        file.write_all(
            format!(
                "\n//------------------------- {} -------------------------\n",
                i
            )
            .as_bytes(),
        )?;

        for j in MINIMUM..=MAXIMUM {
            let res = i - j;
            if res >= MINIMUM && res <= MAXIMUM {
                if i == 0 {
                    file.write_all(
                        format!(
                            r#"
impl<P1, P2> DivScale<Scale<P2, {}>> for Scale<P1, 0>
where
    P1: Prefix,
    P2: Prefix,
{{
    type Output = Scale<P2, {}>;
}}
"#,
                            j, res
                        )
                        .as_bytes(),
                    )?;
                } else if j == 0 {
                    file.write_all(
                        format!(
                            r#"
impl<P1, P2> DivScale<Scale<P2, 0>> for Scale<P1, {}>
where
    P1: Prefix,
    P2: Prefix,
{{
    type Output = Scale<P1, {}>;
}}
"#,
                            i, res
                        )
                        .as_bytes(),
                    )?;
                } else if res == 0 {
                    file.write_all(
                        format!(
                            r#"
impl<P> DivScale<Scale<P, {}>> for Scale<P, {}>
where
    P: Prefix,
{{
    type Output = Scale<One, 0>;
}}
"#,
                            j, i,
                        )
                        .as_bytes(),
                    )?;
                } else {
                    file.write_all(
                        format!(
                            r#"
impl<P> DivScale<Scale<P, {}>> for Scale<P, {}>
where
    P: Prefix,
{{
    type Output = Scale<P, {}>;
}}
"#,
                            j, i, res
                        )
                        .as_bytes(),
                    )?;
                }
            }
        }
    }

    Ok(())
}

pub fn generate_scale_pow() -> Result<()> {
    let mut file = File::create("src/quantity/scale_pow.rs")?;
    file.write_all(b"use super::*;\n\n")?;

    for i in MINIMUM..=MAXIMUM {
        file.write_all(
            format!(
                "\n//------------------------- {} -------------------------\n",
                i
            )
            .as_bytes(),
        )?;

        for j in MINIMUM..=MAXIMUM {
            let res = i * j;
            if res >= MINIMUM && res <= MAXIMUM {
                if res == 0 {
                    file.write_all(
                        format!(
                            r#"
impl<P> PowScale<{}> for Scale<P, {}>
where
    P: Prefix,
{{
    type Output = Scale<One, 0>;
}}
"#,
                            j, i,
                        )
                        .as_bytes(),
                    )?;
                } else {
                    file.write_all(
                        format!(
                            r#"
impl<P> PowScale<{}> for Scale<P, {}>
where
    P: Prefix,
{{
    type Output = Scale<P, {}>;
}}
"#,
                            j, i, res
                        )
                        .as_bytes(),
                    )?;
                }
            }
        }
    }

    Ok(())
}

pub fn generate_scale_root() -> Result<()> {
    let mut file = File::create("src/quantity/scale_root.rs")?;
    file.write_all(b"use super::*;\n\n")?;

    for i in MINIMUM..=MAXIMUM {
        file.write_all(
            format!(
                "\n//------------------------- {} -------------------------\n",
                i
            )
            .as_bytes(),
        )?;

        for j in MINIMUM..=MAXIMUM {
            if j != 0 && i % j == 0 {
                let res = i / j;
                if res >= MINIMUM && res <= MAXIMUM {
                    if res == 0 {
                        file.write_all(
                            format!(
                                r#"
impl<P> RootScale<{}> for Scale<P, {}>
where
    P: Prefix,
{{
    type Output = Scale<One, 0>;
}}
"#,
                                j, i,
                            )
                            .as_bytes(),
                        )?;
                    } else {
                        file.write_all(
                            format!(
                                r#"
impl<P> RootScale<{}> for Scale<P, {}>
where
    P: Prefix,
{{
    type Output = Scale<P, {}>;
}}
"#,
                                j,
                                i,
                                i / j
                            )
                            .as_bytes(),
                        )?;
                    }
                }
            }
        }
    }

    Ok(())
}
