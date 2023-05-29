## Memory profiling of `delta-rs` writer

To run you'll first need a Parquet file handy, for example:

```bash
curl -o supply-chains.parquet https://seafowl-public.s3.eu-west-1.amazonaws.com/tutorial/trase-supply-chains.parquet 
```

Then simply execute the profiling script, passing in the location of the Parquet file:
```bash
./profile.sh /home/ubuntu/supply-chains.parquet
```
This will build the bytehound binaries needed for profiling, run the profiling and then open
a server that provides a UI to investigate the recorded memory profiles.
