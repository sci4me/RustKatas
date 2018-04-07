pub fn insertion_sort<T: Ord>(data: &mut Vec<T>) {
	let mut i = 1;
	while i < data.len() {
		let mut j = i;
		while j > 0 && &data[j - 1] > &data[j] {
			data.swap(j, j - 1);
			j -= 1;
		}
		i += 1;
	}
}

pub fn selection_sort<T: Ord>(data: &mut Vec<T>) {
	let n = data.len();
	for j in 0..n-1 {
		let mut i_min = j;
		
		for i in j+1..n {
			if &data[i] < &data[i_min] {
				i_min = i;
			}
		}

		if i_min != j {
			data.swap(j, i_min);
		}
	}
}

pub fn bubble_sort<T: Ord>(data: &mut Vec<T>) {
	let mut n = data.len();
	loop {
		let mut k = 0;
		for i in 1..n {
			if &data[i - 1] > &data[i] {
				data.swap(i - 1, i);
				k = i;
			}
		}
		n = k;

		if n == 0 {
			break;
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_insertion_sort() {
		let mut a = vec![4, 2, 10, 5, 0, 6, 2, 12, 2, 1, 6];
		insertion_sort(&mut a);
		assert_eq!(a, vec![0, 1, 2, 2, 2, 4, 5, 6, 6, 10, 12]);
	}

	#[test]
	fn test_selection_sort() {
		let mut a = vec![4, 2, 10, 5, 0, 6, 2, 12, 2, 1, 6];
		selection_sort(&mut a);
		assert_eq!(a, vec![0, 1, 2, 2, 2, 4, 5, 6, 6, 10, 12]);
	}


	#[test]
	fn test_bubble_sort() {
		let mut a = vec![4, 2, 10, 5, 0, 6, 2, 12, 2, 1, 6];
		bubble_sort(&mut a);
		assert_eq!(a, vec![0, 1, 2, 2, 2, 4, 5, 6, 6, 10, 12]);
	}
}