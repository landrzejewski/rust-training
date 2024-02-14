
pub fn get_arguments() -> Vec<String> {
    env::args()
    .skip(1)
    .collect::<Vec<_>>()
}