import 'dart:io';

void main() {
  // Read input file
  var raw_input = File('in.txt').readAsStringSync();

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
}
