fn downsample() {
    let mut elevation_data = elevation_iter("elevation_data/full_data");
}

fn elevation_iter(path: &str) -> elevation::ElevationIterator<BufReader<File>> {
    elevation::ElevationIterator(BufReader::new(File::open(path).unwrap()))
}
