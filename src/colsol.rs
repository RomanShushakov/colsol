use std::cmp::min;
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};


pub fn factorization<V>(a: &mut [V], nn: i64, maxa: &[i64]) -> Result<(), String>
    where V: Copy + Debug + From<f32> + PartialOrd + Div<Output=V> + Mul<Output=V> + Add<Output=V> +
             Sub<Output=V>
{
    for n in 0..nn
    {
        let kn = maxa[n as usize];
        let kl = kn + 1;
        let ku = maxa[(n + 1) as usize] - 1;
        let kh = ku - kl;
        if kh < 0
        {
            if  a[kn as usize] <= V::from(0f32)
            {
                let error_message = format!("STOP - STIFFNESS MATRIX NOT POSITIVE \
                    DEFINITE, NONPOSITIVE PIVOT FOR EQUATION {:?}, PIVOT = {:?}",
                    n, a[kn as usize]);
                return Err(error_message);
            }
            else
            {
                continue;
            }
        }
        else if kh == 0
        {
            let mut k = n;
            let mut b = V::from(0f32);
            for kk in kl..=ku
            {
                k = k - 1;
                let ki = maxa[k as usize];
                let c = a[kk as usize] / a[ki as usize];
                b = b + c * a[kk as usize];
                a[kk as usize] = c;
            }
            a[kn as usize] = a[kn as usize] - b;
        }
        else
        {
            let mut k = n - kh;
            let mut ic = 0;
            let mut klt = ku;
            for _j in 1..=kh
            {
                ic = ic + 1;
                klt = klt - 1;
                let ki = maxa[k as usize];
                let nd = maxa[(k + 1) as usize] - ki - 1;
                if nd <= 0
                {
                    k = k + 1;
                    continue;
                }
                else
                {
                    let kk = min(ic, nd);
                    let mut c = V::from(0f32);
                    for l in 1..=kk
                    {
                        c = c + a[(ki + l) as usize] * a[(klt + l) as usize];
                    }
                    a[klt as usize] = a[klt as usize] - c;
                    k = k + 1;
                    continue;
                }
            }

            let mut b = V::from(0f32);
            for kk in kl..=ku
            {
                k = k - 1;
                let ki = maxa[k as usize];
                let c = a[kk as usize] / a[ki as usize];
                b = b + c * a[kk as usize];
                a[kk as usize] = c;
            }
            a[kn as usize] = a[kn as usize] - b;
        }
        continue;
    }
    Ok(())
}


pub fn find_unknown<V>(a: &[V], v: &mut[V], nn: i64, maxa: &[i64])
    where V: Copy + Mul<Output=V> + Add<Output=V> + Sub<Output=V> + Div<Output=V> + From<f32>
{
    // reduce rhs vector

    for n in 0..nn
    {
        let kl = maxa[n as usize] + 1;
        let ku = maxa[(n + 1) as usize] - 1;
        if ku - kl < 0
        {
            continue;
        }
        else
        {
            let mut k = n;
            let mut c = V::from(0f32);
            for kk in kl..=ku
            {
                k = k - 1;
                c = c + a[kk as usize] * v[k as usize];
            }
            v[n as usize] = v[n as usize] - c;
        }
        continue;
    }


    // back-substitute

    for n in 0..nn
    {
        let k = maxa[n as usize];
        v[n as usize] = v[n as usize] / a[k as usize];
    }

    if nn == 0
    {
        return;
    }
    let mut n = nn - 1;

    for _l in 1..=nn
    {
        let kl = maxa[n as usize] + 1;
        let ku = maxa[(n + 1) as usize] - 1;
        if ku - kl < 0
        {
            n = n - 1;
        }
        else
        {
            let mut k = n;
            for kk in kl..=ku
            {
                k = k - 1;
                v[k as usize] = v[k as usize] - a[kk as usize] * v[n as usize];
            }
            n = n - 1;
        }
    }
}
