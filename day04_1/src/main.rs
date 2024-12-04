fn xmas_count(input: &[Vec<char>], line_size: usize) -> usize {
    let mut count = 0;
    for i in 0..input.len() {
        assert_eq!(input[i].len(), line_size);
        for j in 0..line_size {
            if input[i][j] != 'X' {
                continue;
            }
        }
    }
    count
}

fn main() {
    println!("Hello, world!");
}
