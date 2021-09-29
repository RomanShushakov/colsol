use std::cmp::min;


fn factorization(a: &mut [f64], nn: i32, maxa: &[i32])
{
    for n in 0..nn
    {
        let kn = maxa[n as usize];
        let kl = kn + 1;
        let ku = maxa[(n + 1) as usize] - 1;
        let kh = ku - kl;
        if kh < 0
        {
            if  a[kn as usize] <= 0f64
            {
                let error_message = &format!("STOP - STIFFNESS MATRIX NOT POSITIVE \
                    DEFINITE, NONPOSITIVE PIVOT FOR EQUATION {:?}, PIVOT = {:?}",
                    n, a[kn as usize]);
                println!("{}", error_message);
                return;
            }
            else
            {
                continue;
            }
        }
        else if kh == 0
        {
            let mut k = n;
            let mut b = 0f64;
            for kk in kl..=ku
            {
                k = k - 1;
                let ki = maxa[k as usize];
                let c = a[kk as usize] / a[ki as usize];
                b = b + c * a[kk as usize];
                a[kk as usize] = c;
            }
            a[kn as usize] = a[kn as usize] - b;

            if  a[kn as usize] <= 0f64
            {
                let error_message = &format!("STOP - STIFFNESS MATRIX NOT POSITIVE \
                    DEFINITE, NONPOSITIVE PIVOT FOR EQUATION {:?}, PIVOT = {:?}",
                    n, a[kn as usize]);
                println!("{}", error_message);
                return;
            }
            else
            {
                continue;
            }
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
                }
                else
                {
                    let kk = min(ic, nd);
                    let mut c = 0.0;
                    for l in 1..=kk
                    {
                        c = c + a[(ki + l) as usize] * a[(klt + l) as usize];
                    }
                    a[klt as usize] = a[klt as usize] - c;

                    k = k + 1;

                    let mut k = n;
                    let mut b = 0f64;
                    for kk in kl..=ku
                    {
                        k = k - 1;
                        let ki = maxa[k as usize];
                        let c = a[kk as usize] / a[ki as usize];
                        b = b + c * a[kk as usize];
                        a[kk as usize] = c;
                    }
                    a[kn as usize] = a[kn as usize] - b;

                    if  a[kn as usize] <= 0f64
                    {
                        let error_message = &format!("STOP - STIFFNESS MATRIX NOT POSITIVE \
                            DEFINITE, NONPOSITIVE PIVOT FOR EQUATION {:?}, PIVOT = {:?}",
                            n, a[kn as usize]);
                        println!("{}", error_message);
                        return;
                    }
                    else
                    {
                        continue;
                    }

                }
                k = k + 1;
            }
        }
        continue;
    }
}


fn find_displacements(a: &[f64], v: &mut[f64], nn: i32, maxa: &[i32])
{
    // reduce rhs load vector

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
            let mut c = 0.0;
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

    for l in 1..=nn
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


fn main()
{
    let nn = 4i32;
    let mut a = vec![5.0, 6.0, -4.0, 6.0, -4.0, 1.0, 5.0, -4.0, 1.0];
    let maxa = vec![0i32, 1, 3, 6, 9];
    let mut v = vec![0.0, 1.0, 0.0, 0.0];
    factorization(&mut a, nn, &maxa);
    println!("a: {:?}", a);
    find_displacements(&a, &mut v, nn, &maxa);
    println!("v: {:?}", v);
}
