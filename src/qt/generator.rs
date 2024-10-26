use std::fs::File;
use std::io::Result;
use std::io::Write;

pub fn generate_exponent_mul() -> Result<()> {
    let mut file = File::create("src/qt/exponent_mul.rs")?;
    file.write_all(b"use super::*;\n\n")?;

    let first = -10;
    let last = 10;

    for i in first..=last {
        file.write_all(
            format!(
                "\n//------------------------- {} -------------------------\n",
                i
            )
            .as_bytes(),
        )?;

        for j in first..=last {
            file.write_all(
                format!(
                    r#"
impl MulExp<Exp<{}>> for Exp<{}> {{
    type Output = Exp<{}>;
}}
            "#,
                    j,
                    i,
                    i + j
                )
                .as_bytes(),
            )?;
        }
    }

    Ok(())
}

pub fn generate_exponent_div() -> Result<()> {
    let mut file = File::create("src/qt/exponent_div.rs")?;
    file.write_all(b"use super::*;\n\n")?;

    let first = -10;
    let last = 10;

    for i in first..=last {
        file.write_all(
            format!(
                "\n//------------------------- {} -------------------------\n",
                i
            )
            .as_bytes(),
        )?;

        for j in first..=last {
            file.write_all(
                format!(
                    r#"
impl DivExp<Exp<{}>> for Exp<{}> {{
    type Output = Exp<{}>;
}}
            "#,
                    j,
                    i,
                    i - j
                )
                .as_bytes(),
            )?;
        }
    }

    Ok(())
}

pub fn generate_exponent_pow() -> Result<()> {
    let mut file = File::create("src/qt/exponent_pow.rs")?;
    file.write_all(b"use super::*;\n\n")?;

    let first = -10;
    let last = 10;

    for i in first..=last {
        file.write_all(
            format!(
                "\n//------------------------- {} -------------------------\n",
                i
            )
            .as_bytes(),
        )?;

        for j in first..=last {
            file.write_all(
                format!(
                    r#"
impl PowExp<{}> for Exp<{}> {{
    type Output = Exp<{}>;
}}
            "#,
                    j,
                    i,
                    i * j
                )
                .as_bytes(),
            )?;
        }
    }

    Ok(())
}

pub fn generate_exponent_root() -> Result<()> {
    let mut file = File::create("src/qt/exponent_root.rs")?;
    file.write_all(b"use super::*;\n\n")?;

    let first: i32 = -10;
    let last: i32 = 10;

    for i in first..=last {
        file.write_all(
            format!(
                "\n//------------------------- {} -------------------------\n",
                i
            )
            .as_bytes(),
        )?;

        for j in first..=last {
            // if j != 0 {
            //     println!("{} % {} = {}", i, j, i % j);
            // }
            if j != 0 && i % j == 0 {
                file.write_all(
                    format!(
                        r#"
impl RootExp<{}> for Exp<{}> {{
    type Output = Exp<{}>;
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

    Ok(())
}
