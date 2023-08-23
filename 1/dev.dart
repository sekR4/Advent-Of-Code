import 'dart:io';

void main() {
  // Read input file
  var raw_input = File('in.txt').readAsStringSync();

  // Part 1
  int max_sum = 0;
  for (var elf in raw_input.split('\n\n')) {
    // get current sum
    int curr_sum = 0;
    for (var package in elf.split('\n')) {
      // assure that package is not empty
      if (package.isNotEmpty) {
        curr_sum += int.parse(package);
      }
    }

    // update max sum
    if (curr_sum > max_sum) {
      max_sum = curr_sum;
    }
  }
  // print maximum sum
  print('The strongest elf carries $max_sum calories.');

  // Part 2
  List list_of_sums = [];

  // 1. Loop through raw input (\n\n) & append the sum of each `elf` to list_of_sums
  for (var elf in raw_input.split('\n\n')) {
    // get current sum
    int curr_sum = 0;
    for (var package in elf.split('\n')) {
      // assure that package is not empty
      if (package.isNotEmpty) {
        curr_sum += int.parse(package);
      }
    }
    // append current sum to list_of_sums
    list_of_sums.add(curr_sum);
  }

  // 2. Sort list_of_sums descending
  list_of_sums.sort((a, b) => b.compareTo(a));

  // 3. Build the sum of the top 3 items of list_of_sums
  num sum_of_top_3 = 0;
  for (int i = 0; i < 3; i++) {
    sum_of_top_3 += list_of_sums[i];
  }

  // print the sum of the top 3 items of list_of_sums
  print('The top 3 elfs are carrying $sum_of_top_3 calories.');
}
