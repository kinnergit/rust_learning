pub fn create_box(i: i32)
{
    let _x = Box::new(i);
}

pub fn create_box_with_range(start: i32, end: i32)
{
    for i in start .. end
    {
        create_box(i)
    }
}
