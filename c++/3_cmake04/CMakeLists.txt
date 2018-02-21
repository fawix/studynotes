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
