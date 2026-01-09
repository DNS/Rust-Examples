
<#
https://goperf.dev/01-common-patterns/comp-flags/
https://zderadicka.eu/static-build-of-rust-executables/
#>

gcc -static -O3 benchmark.c -o benchmark-c.out
rustc -C opt-level=3 -C strip=symbols -C target-feature=+crt-static benchmark.rs -o benchmark-rust.out
#go build -ldflags="-s -w -extldflags '-static'" -o benchmark-go.out benchmark.go
go build -ldflags="-s -w -linkmode=external -extldflags '-static'" -o benchmark-go.out benchmark.go



# benchmark, find fastest time
gci *.out | %{ '---------'; $_.Name; ldd $_ }
'---------'
gci *.out | %{ "$($_.Name) :  $(iex $_)" }

'---------'

$c = Measure-Command { ./benchmark-c.out }
$r = Measure-Command { ./benchmark-rust.out }
$g = Measure-Command { ./benchmark-go.out }

'Total Seconds'
"C    : $($c.TotalSeconds)"
"Rust : $($r.TotalSeconds)"
"Go   : $($g.TotalSeconds)"


ri *.out -Force
