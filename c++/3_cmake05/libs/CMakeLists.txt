# The below adds a library in this folder
# (which we told cmake about on the top level CMakeLists.txt)
# cmake will assume a header file with same name as our library is present.
add_library(libs mysqrt.cpp)

install (TARGETS libs DESTINATION bin)
install (FILES mysqrt.h DESTINATION include)
