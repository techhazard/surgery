use std::collections::VecDeque;
use std::io::BufRead;

pub fn decapitate(source: &mut dyn BufRead, amount_of_lines_to_skip: usize) -> std::io::Result<()> {
    let mut lines_iter = source.lines().map(|line| line.unwrap() ).skip(amount_of_lines_to_skip);

    while let Some(newest_line) = lines_iter.next() {
      println!("{}", newest_line);
    }
    Ok(())
}


pub fn amputate(source: &mut dyn BufRead, amount_of_lines_to_skip: usize) -> std::io::Result<()> {
    let mut lines_iter = source.lines().map(|line| line.unwrap() );
    let mut buffer : VecDeque<String> = lines_iter.by_ref().take(amount_of_lines_to_skip).collect::<VecDeque<String>>();

    while let Some(newest_line) = lines_iter.next() {
      let oldest_line = buffer.pop_front();
      buffer.push_back(newest_line);
      println!("{}", oldest_line.unwrap());
    }

    Ok(())
}


pub fn head(source: &mut dyn BufRead, amount_of_lines_to_keep: usize) -> std::io::Result<()> {
    let mut lines_iter = source.lines().map(|line| line.unwrap()).take(amount_of_lines_to_keep);
    while let Some(newest_line) = lines_iter.next() {
        println!("{}", newest_line);
    }
    Ok(())
}


pub fn tail(source: &mut dyn BufRead, amount_of_lines_to_keep: usize) -> std::io::Result<()> {
    let mut lines_iter = source.lines().map(|line| line.unwrap() );
    let mut buffer : VecDeque<String> = lines_iter.by_ref().take(amount_of_lines_to_keep).collect::<VecDeque<String>>();

    while let Some(newest_line) = lines_iter.next() {
      let oldest_line = buffer.pop_front();
      buffer.push_back(newest_line);
    }
    let mut buffer_iter = buffer.iter();

    while let Some(line) = buffer_iter.next() {
        println!("{}", line);
    }

    Ok(())

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
