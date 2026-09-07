fn main() {
    let mut x = 1;
    {
        let _immut_x_1 = &x;
    }

    {
        let _mut_x_1 = &mut x;
    }

    let _mut_x_2 = &mut x;
    let _immut_x_3 = &x;
}
