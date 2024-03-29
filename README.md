# awesome-leetcode-rs

This is a repo for well-written leetcode solutions. Solutions to the problems are put in the problems directory with unit tests. File is named after the leetcode problem id.

For example, the url below maps to `problems/two_sum`
> https://leetcode.com/problems/two-sum/

After you added a new solution, clean the build cache so autocmd would pick up the new file
> cargo clean

To run all tests, do
> cargo test problems

To run it for a specific problem, do
> cargo test problems::two_sum
