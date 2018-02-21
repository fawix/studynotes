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
 
# add the executable
add_executable (sqroot_n sqroot.cpp)


# In this cmake file we still haev our flat structure but now we are passing some of 
# our build parameters to our program
# namely the version :) 
# Go check the SqrootConfig.h file
