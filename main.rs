fn arraysort(arr: &mut [usize], size: &mut usize) {
    let mut temp: usize;

    let idk = *size - 1;

    for i in 0..idk {
        let another = *size - i - 1;

        for j in 0..another {
            if arr[j] > arr[j + 1] {
                temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
                arr[j];
            }
        }
    }
}

fn main() {
    let mut arr = [4, 5, 7, 1, 9, 6];
    let mut size: usize = arr.len();

    arraysort(&mut arr, &mut size);

    println!("{:?}", arr);
}
