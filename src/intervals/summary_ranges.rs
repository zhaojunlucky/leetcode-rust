fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    if nums.len() == 0 {

        return vec![]
    }
    let mut i = 0;
    let mut j = 1;

    let mut ans = vec![];

    loop  {
        if j >= nums.len() || nums[j] != nums[j-1] + 1 {
            // i..j-1
            if i == j - 1 {
                ans.push(nums[i].to_string());
            } else {
                ans.push(format!("{}->{}", nums[i], nums[j - 1]));
            }

            if j >= nums.len() {
                break;
            }

            i = j;
        }
        j += 1
    }

    ans
}