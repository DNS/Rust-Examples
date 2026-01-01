rustc -C opt-level=3 -C strip=symbols benchmark.rs -o benchmark-rust.out

gcc -O3 benchmark.c -o benchmark-c.out


# benchmark, find fastest time

$c = Measure-Command { ./benchmark-c.out }
$r = Measure-Command { ./benchmark-rust.out }


ri *.out -Force

'Total Seconds'
"C    : $($c.TotalSeconds)"
"Rust : $($r.TotalSeconds)"
