use pyo3::{exceptions::PyValueError, PyResult};
use rug::{ops::Pow, Float, Integer};

fn binary_split(a: u32, b: u32) -> (Integer, Integer, Integer) {
    if b - a == 1 {
        if a == 0 {
            let pab = Integer::from(1);
            let qab = Integer::from(1);
            let rab = Integer::from(&pab * (13591409 + 545140134 * a));
            return (pab, qab, rab);
        }
        let a_bigint = Integer::from(a);
        let pab: Integer = (Integer::from(6 * &a_bigint) - 5)
            * (Integer::from(2 * &a_bigint) - 1)
            * (Integer::from(6 * &a_bigint) - 1);
        let qab = a_bigint.clone().pow(3) * 10939058860032000u64;
        let rab = &pab * (13591409 + 545140134 * a_bigint);

        if a % 2 == 0 {
            return (pab, qab, rab);
        }
        return (pab, qab, -1 * rab);
    }
    let m = (a + b) / 2;
    let (pam, qam, ram) = binary_split(a, m);
    let (pmb, qmb, rmb) = binary_split(m, b);
    let p1n = Integer::from(&pam * &pmb);
    let q1n = Integer::from(&qam * &qmb);
    let r1n = Integer::from(&ram * &qmb) + Integer::from(&pam * &rmb);
    (p1n, q1n, r1n)
}

pub fn chudnovsky(digits: u32) -> PyResult<String> {
    match digits {
        0 => {
            return Err(PyValueError::new_err(
                "Invalid digits: must be greater than 0",
            ))
        }
        1 => return Ok("3.1".to_string()),
        _ => {
            if digits.checked_mul(4).is_none() {
                return Err(PyValueError::new_err(
                    "Invalid digits: must be less than (2^32-1)/4",
                ));
            }
        }
    }
    let used_precision = digits * 4;
    let digits_per_term = f32::log10(10939058860032000f32 / 6f32 / 2f32 / 6f32);
    let n = (digits as f32 / digits_per_term).ceil() as u32;
    let i1 = Integer::from(426880);
    let i2 = Float::with_val(used_precision, 10005);

    let (_, q1n, r1n) = binary_split(0, n);
    Ok(((i1 * i2.sqrt() * q1n) / r1n).to_string())
}
