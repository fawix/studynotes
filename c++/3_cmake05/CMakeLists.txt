cmake_minimum_required (VERSION 2.6)
project (sqroot)

#Adding version number v1.0:
set (sqroot_VERSION_MAJOR 1)
set (sqroot_VERSION_MINOR 0)

# configure a header file to pass some of the CMake settings
# to the source code
configure_file (
  "${PROJECT_SOURCE_DIR}/SqrootConfig.h.in"
  "${PROJECT_BINARY_DIR}/SqrootConfig.h"
  )

# add the binary tree to the search path for include files
# so that we will find our config header
include_directories("${PROJECT_BINARY_DIR}")

# should we use our own math functions?
option (USE_MYMATH 
	"Use custom sqroot" ON)
# NOTE:
# This will show up in the CMake GUI with a default value of ON that 
# the user can change as desired. This setting will be stored in the 
# cache so that the user does not need to keep setting it 
# each time they run CMake on this project.
# Although I don't really use the GUI... 


#
# Now we need to make our libs conditional add
#
if (USE_MYMATH)
   # To make use of our new library we need to tell CMake to compile it
   # this two lines will include teh directory and sub directory to the 
   # cmake list
   include_directories ("${PROJECT_SOURCE_DIR}/libs")
   add_subdirectory (libs)

   set (EXTRA_LIBS ${EXTRA_LIBS} libs)
endif (USE_MYMATH)
 
# add the executable
add_executable (sqroot_n sqroot.cpp)

#This tells cmake to use our library to build our executable.
target_link_libraries (sqroot_n ${EXTRA_LIBS})

# with this we modify our sqroot.cpp in order to 
# optionally use the custom implementation of sqrt


# add the install targets
install (TARGETS sqroot_n DESTINATION bin)
install (FILES "${PROJECT_BINARY_DIR}/SqrootConfig.h"        
         DESTINATION include)



# Let's add some tests too..
# Just for demonstration
# Although this might not be the best way 
# to approach TDD / BDD or even unit testing :) 
include(CTest)

# does the application run
add_test (SqrootRun sqroot_n 25)

# does it sqrt of 25
add_test (SqrootComp25 sqroot_n 25)
set_tests_properties (SqrootComp25 PROPERTIES PASS_REGULAR_EXPRESSION "25 is 5")

# does it handle negative numbers
add_test (SqrootNegative sqroot_n -25)
set_tests_properties (SqrootNegative PROPERTIES PASS_REGULAR_EXPRESSION "-25 is 0")

# does it handle small numbers
add_test (SqrootSmall sqroot_n 0.0001)
set_tests_properties (SqrootSmall PROPERTIES PASS_REGULAR_EXPRESSION "0.0001 is 0.01")

# does the usage message work?
add_test (SqrootUsage sqroot_n)
set_tests_properties (SqrootUsage PROPERTIES PASS_REGULAR_EXPRESSION "Usage:.*number")


# Or.. define a macro to simplify adding tests, then use it
macro (do_test arg result)
  add_test (SqrootComp${arg} sqroot_n ${arg})
  set_tests_properties (SqrootComp${arg}
    PROPERTIES PASS_REGULAR_EXPRESSION ${result})
endmacro (do_test)
 
# do a bunch of result based tests
do_test (25 "25 is 5")
do_test (-25 "-25 is 0")


