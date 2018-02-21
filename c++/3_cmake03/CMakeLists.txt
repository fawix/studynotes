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

# To make use of our new library we need to tell CMake to compile it
# this two lines will include teh directory and sub directory to the 
# cmake list
include_directories ("${PROJECT_SOURCE_DIR}/libs")
add_subdirectory (libs) 

# add the executable
add_executable (sqroot_n sqroot.cpp)

#This tells cmake to use our library to build our executable.
target_link_libraries (sqroot_n libs)

# Now with this we have a new libs folder that contain our libraries. 

