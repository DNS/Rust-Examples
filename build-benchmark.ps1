rustc -C opt-level=3 -C strip=symbols benchmark.rs -o benchmark-rust.out

gcc -O3 benchmark.c -o benchmark-c.out


# benchmark, find fastest time

$c,$r = @(100,100)

$c = $(Measure-Command { ./benchmark-c.out }).TotalSeconds
$r = $(Measure-Command { ./benchmark-rust.out }).TotalSeconds 


ri *.out -Force

'Total Seconds'
"C    : $($c.TotalSeconds)"
"Rust : $($r.TotalSeconds)"
