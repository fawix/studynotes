# CMake generated Testfile for 
# Source directory: /home/fawix/workspace/c_build_learn/3_cmake06
# Build directory: /home/fawix/workspace/c_build_learn/3_cmake06
# 
# This file includes the relevant testing commands required for 
# testing this directory and lists subdirectories to be tested as well.
add_test(SqrootRun "sqroot_n" "25")
add_test(SqrootComp25 "sqroot_n" "25")
set_tests_properties(SqrootComp25 PROPERTIES  PASS_REGULAR_EXPRESSION "25 is 5")
add_test(SqrootNegative "sqroot_n" "-25")
set_tests_properties(SqrootNegative PROPERTIES  PASS_REGULAR_EXPRESSION "-25 is 0")
add_test(SqrootSmall "sqroot_n" "0.0001")
set_tests_properties(SqrootSmall PROPERTIES  PASS_REGULAR_EXPRESSION "0.0001 is 0.01")
add_test(SqrootUsage "sqroot_n")
set_tests_properties(SqrootUsage PROPERTIES  PASS_REGULAR_EXPRESSION "Usage:.*number")
add_test(SqrootComp-25 "sqroot_n" "-25")
set_tests_properties(SqrootComp-25 PROPERTIES  PASS_REGULAR_EXPRESSION "-25 is 0")
subdirs("libs")
