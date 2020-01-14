use std::collections::VecDeque;
use std::io;
use std::io::BufRead;

pub fn decapitate(source: &mut dyn io::BufRead, amount_of_lines_to_skip: usize) -> io::Result<usize> {
    let mut lines_iter = source.lines().map(|line| line.unwrap() ).skip(amount_of_lines_to_skip);
    let mut printed_lines : usize = 0;

    while let Some(newest_line) = lines_iter.next() {
        println!("{}", newest_line);
        printed_lines += 1;
    }
    Ok(printed_lines)
}


pub fn amputate(source: &mut dyn io::BufRead, amount_of_lines_to_skip: usize) -> io::Result<usize> {
    let mut lines_iter = source.lines().map(|line| line.unwrap() );
    let mut buffer : VecDeque<String> = lines_iter.by_ref().take(amount_of_lines_to_skip).collect::<VecDeque<String>>();
    let mut printed_lines : usize = 0;

    while let Some(newest_line) = lines_iter.next() {
        let oldest_line = buffer.pop_front();
        buffer.push_back(newest_line);
        if let Some(line) = oldest_line {
            println!("{}", line);
            printed_lines += 1;
        }
    }
    Ok(printed_lines)
}


pub fn head(source: &mut dyn io::BufRead, amount_of_lines_to_keep: usize) -> io::Result<usize> {
    let mut lines_iter = source.lines().map(|line| line.unwrap()).take(amount_of_lines_to_keep);
    let mut printed_lines : usize = 0;
    while let Some(newest_line) = lines_iter.next() {
        println!("{}", newest_line);
        printed_lines += 1;
    }
    Ok(printed_lines)
}


pub fn tail(source: &mut dyn io::BufRead, amount_of_lines_to_keep: usize) -> io::Result<usize> {
    let mut lines_iter = source.lines().map(|line| line.unwrap() );
    let mut buffer : VecDeque<String> = lines_iter.by_ref().take(amount_of_lines_to_keep).collect::<VecDeque<String>>();
    let mut printed_lines : usize = 0;

    while let Some(newest_line) = lines_iter.next() {
      let _ = buffer.pop_front();
      buffer.push_back(newest_line);
    }

    let mut buffer_iter = buffer.iter();
    while let Some(line) = buffer_iter.next() {
        println!("{}", line);
        printed_lines += 1;
    }
    Ok(printed_lines)
}


#[cfg(test)]
mod tests {
    use std::io;

    #[test]
    fn amputate_default() {
        let amount_of_lines_to_skip = 10;

        let mut source = io::Cursor::new(b"");
        let output = super::amputate(&mut source, amount_of_lines_to_skip);
        assert_eq!(output.unwrap(), 0);

        let mut source = io::Cursor::new(b"1\n2\n3\n4\n5\n6\n7\n8\n9");
        let output = super::amputate(&mut source, amount_of_lines_to_skip);
        assert_eq!(output.unwrap(), 0);

        let mut source = io::Cursor::new(b"1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11");
        let output = super::amputate(&mut source, amount_of_lines_to_skip);
        assert_eq!(output.unwrap(), 1);

        let mut source = io::Cursor::new(b"1\n2\n3\n4\n5\n6\n7\n8\n9\n10");
        let output = super::amputate(&mut source, amount_of_lines_to_skip);
        assert_eq!(output.unwrap(), 0);
    }

    #[test]
    fn amputate_zero() {
        let amount_of_lines_to_skip = 0;

        let mut source = io::Cursor::new(b"");
        let output = super::amputate(&mut source, amount_of_lines_to_skip);
        assert_eq!(output.unwrap(), 0);

        let mut source = io::Cursor::new(b"1\n2\n3\n4\n5\n6\n7\n8\n9");
        let output = super::amputate(&mut source, amount_of_lines_to_skip);
        assert_eq!(output.unwrap(), 8);

        let mut source = io::Cursor::new(b"1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11");
        let output = super::amputate(&mut source, amount_of_lines_to_skip);
        assert_eq!(output.unwrap(), 10);

        let mut source = io::Cursor::new(b"1\n2\n3\n4\n5\n6\n7\n8\n9\n10");
        let output = super::amputate(&mut source, amount_of_lines_to_skip);
        assert_eq!(output.unwrap(), 9);
    }

    #[test]
    fn amputate_one() {
        let amount_of_lines_to_skip = 1;

        let mut source = io::Cursor::new(b"");
        let output = super::amputate(&mut source, amount_of_lines_to_skip);
        assert_eq!(output.unwrap(), 0);

        let mut source = io::Cursor::new(b"1\n2\n3\n4\n5\n6\n7\n8\n9");
        let output = super::amputate(&mut source, amount_of_lines_to_skip);
        assert_eq!(output.unwrap(), 8);

        let mut source = io::Cursor::new(b"1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11");
        let output = super::amputate(&mut source, amount_of_lines_to_skip);
        assert_eq!(output.unwrap(), 10);

        let mut source = io::Cursor::new(b"1\n2\n3\n4\n5\n6\n7\n8\n9\n10");
        let output = super::amputate(&mut source, amount_of_lines_to_skip);
        assert_eq!(output.unwrap(), 9);
    }

}
