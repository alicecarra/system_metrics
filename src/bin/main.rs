fn main() {
    let memory = system_metrics::memory::get_memory();

    println!("{memory:#?}");
}
