pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut l = 0;
    for (i, item) in nums.clone().iter().enumerate(){
        if nums[i] != 0 {
            nums.swap(i, l);
            l += 1;
        }
    }
}

#[cfg(test)]
mod test{
    use super::move_zeroes;

    #[test]
    fn should_return_true(){
        let mut nums = vec![0,1,0,3,12];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![1,3,12,0,0]);
    }
}