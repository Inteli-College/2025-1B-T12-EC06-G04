# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.28

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:

#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:

# Disable VCS-based implicit rules.
% : %,v

# Disable VCS-based implicit rules.
% : RCS/%

# Disable VCS-based implicit rules.
% : RCS/%,v

# Disable VCS-based implicit rules.
% : SCCS/s.%

# Disable VCS-based implicit rules.
% : s.%

.SUFFIXES: .hpux_make_needs_suffix_list

# Command-line flag to silence nested $(MAKE).
$(VERBOSE)MAKESILENT = -s

#Suppress display of executed commands.
$(VERBOSE).SILENT:

# A target that is always out of date.
cmake_force:
.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /usr/bin/cmake

# The command to remove a file.
RM = /usr/bin/cmake -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /home/inteli/Documentos/modulo6/2025-1B-T12-EC06-G04/src

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /home/inteli/Documentos/modulo6/2025-1B-T12-EC06-G04/src/build

# Include any dependencies generated for this target.
include CMakeFiles/validacao.dir/depend.make
# Include any dependencies generated by the compiler for this target.
include CMakeFiles/validacao.dir/compiler_depend.make

# Include the progress variables for this target.
include CMakeFiles/validacao.dir/progress.make

# Include the compile flags for this target's objects.
include CMakeFiles/validacao.dir/flags.make

CMakeFiles/validacao.dir/src/validacao.cpp.o: CMakeFiles/validacao.dir/flags.make
CMakeFiles/validacao.dir/src/validacao.cpp.o: /home/inteli/Documentos/modulo6/2025-1B-T12-EC06-G04/src/src/validacao.cpp
CMakeFiles/validacao.dir/src/validacao.cpp.o: CMakeFiles/validacao.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/home/inteli/Documentos/modulo6/2025-1B-T12-EC06-G04/src/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building CXX object CMakeFiles/validacao.dir/src/validacao.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/validacao.dir/src/validacao.cpp.o -MF CMakeFiles/validacao.dir/src/validacao.cpp.o.d -o CMakeFiles/validacao.dir/src/validacao.cpp.o -c /home/inteli/Documentos/modulo6/2025-1B-T12-EC06-G04/src/src/validacao.cpp

CMakeFiles/validacao.dir/src/validacao.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/validacao.dir/src/validacao.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /home/inteli/Documentos/modulo6/2025-1B-T12-EC06-G04/src/src/validacao.cpp > CMakeFiles/validacao.dir/src/validacao.cpp.i

CMakeFiles/validacao.dir/src/validacao.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/validacao.dir/src/validacao.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /home/inteli/Documentos/modulo6/2025-1B-T12-EC06-G04/src/src/validacao.cpp -o CMakeFiles/validacao.dir/src/validacao.cpp.s

CMakeFiles/validacao.dir/src/classificador.cpp.o: CMakeFiles/validacao.dir/flags.make
CMakeFiles/validacao.dir/src/classificador.cpp.o: /home/inteli/Documentos/modulo6/2025-1B-T12-EC06-G04/src/src/classificador.cpp
CMakeFiles/validacao.dir/src/classificador.cpp.o: CMakeFiles/validacao.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/home/inteli/Documentos/modulo6/2025-1B-T12-EC06-G04/src/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Building CXX object CMakeFiles/validacao.dir/src/classificador.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/validacao.dir/src/classificador.cpp.o -MF CMakeFiles/validacao.dir/src/classificador.cpp.o.d -o CMakeFiles/validacao.dir/src/classificador.cpp.o -c /home/inteli/Documentos/modulo6/2025-1B-T12-EC06-G04/src/src/classificador.cpp

CMakeFiles/validacao.dir/src/classificador.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/validacao.dir/src/classificador.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /home/inteli/Documentos/modulo6/2025-1B-T12-EC06-G04/src/src/classificador.cpp > CMakeFiles/validacao.dir/src/classificador.cpp.i

CMakeFiles/validacao.dir/src/classificador.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/validacao.dir/src/classificador.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /home/inteli/Documentos/modulo6/2025-1B-T12-EC06-G04/src/src/classificador.cpp -o CMakeFiles/validacao.dir/src/classificador.cpp.s

CMakeFiles/validacao.dir/src/preprocessamento.cpp.o: CMakeFiles/validacao.dir/flags.make
CMakeFiles/validacao.dir/src/preprocessamento.cpp.o: /home/inteli/Documentos/modulo6/2025-1B-T12-EC06-G04/src/src/preprocessamento.cpp
CMakeFiles/validacao.dir/src/preprocessamento.cpp.o: CMakeFiles/validacao.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/home/inteli/Documentos/modulo6/2025-1B-T12-EC06-G04/src/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_3) "Building CXX object CMakeFiles/validacao.dir/src/preprocessamento.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/validacao.dir/src/preprocessamento.cpp.o -MF CMakeFiles/validacao.dir/src/preprocessamento.cpp.o.d -o CMakeFiles/validacao.dir/src/preprocessamento.cpp.o -c /home/inteli/Documentos/modulo6/2025-1B-T12-EC06-G04/src/src/preprocessamento.cpp

CMakeFiles/validacao.dir/src/preprocessamento.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/validacao.dir/src/preprocessamento.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /home/inteli/Documentos/modulo6/2025-1B-T12-EC06-G04/src/src/preprocessamento.cpp > CMakeFiles/validacao.dir/src/preprocessamento.cpp.i

CMakeFiles/validacao.dir/src/preprocessamento.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/validacao.dir/src/preprocessamento.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /home/inteli/Documentos/modulo6/2025-1B-T12-EC06-G04/src/src/preprocessamento.cpp -o CMakeFiles/validacao.dir/src/preprocessamento.cpp.s

CMakeFiles/validacao.dir/src/detectorfissura.cpp.o: CMakeFiles/validacao.dir/flags.make
CMakeFiles/validacao.dir/src/detectorfissura.cpp.o: /home/inteli/Documentos/modulo6/2025-1B-T12-EC06-G04/src/src/detectorfissura.cpp
CMakeFiles/validacao.dir/src/detectorfissura.cpp.o: CMakeFiles/validacao.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/home/inteli/Documentos/modulo6/2025-1B-T12-EC06-G04/src/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_4) "Building CXX object CMakeFiles/validacao.dir/src/detectorfissura.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/validacao.dir/src/detectorfissura.cpp.o -MF CMakeFiles/validacao.dir/src/detectorfissura.cpp.o.d -o CMakeFiles/validacao.dir/src/detectorfissura.cpp.o -c /home/inteli/Documentos/modulo6/2025-1B-T12-EC06-G04/src/src/detectorfissura.cpp

CMakeFiles/validacao.dir/src/detectorfissura.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/validacao.dir/src/detectorfissura.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /home/inteli/Documentos/modulo6/2025-1B-T12-EC06-G04/src/src/detectorfissura.cpp > CMakeFiles/validacao.dir/src/detectorfissura.cpp.i

CMakeFiles/validacao.dir/src/detectorfissura.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/validacao.dir/src/detectorfissura.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /home/inteli/Documentos/modulo6/2025-1B-T12-EC06-G04/src/src/detectorfissura.cpp -o CMakeFiles/validacao.dir/src/detectorfissura.cpp.s

CMakeFiles/validacao.dir/src/extrator_features.cpp.o: CMakeFiles/validacao.dir/flags.make
CMakeFiles/validacao.dir/src/extrator_features.cpp.o: /home/inteli/Documentos/modulo6/2025-1B-T12-EC06-G04/src/src/extrator_features.cpp
CMakeFiles/validacao.dir/src/extrator_features.cpp.o: CMakeFiles/validacao.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/home/inteli/Documentos/modulo6/2025-1B-T12-EC06-G04/src/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_5) "Building CXX object CMakeFiles/validacao.dir/src/extrator_features.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/validacao.dir/src/extrator_features.cpp.o -MF CMakeFiles/validacao.dir/src/extrator_features.cpp.o.d -o CMakeFiles/validacao.dir/src/extrator_features.cpp.o -c /home/inteli/Documentos/modulo6/2025-1B-T12-EC06-G04/src/src/extrator_features.cpp

CMakeFiles/validacao.dir/src/extrator_features.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/validacao.dir/src/extrator_features.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /home/inteli/Documentos/modulo6/2025-1B-T12-EC06-G04/src/src/extrator_features.cpp > CMakeFiles/validacao.dir/src/extrator_features.cpp.i

CMakeFiles/validacao.dir/src/extrator_features.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/validacao.dir/src/extrator_features.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /home/inteli/Documentos/modulo6/2025-1B-T12-EC06-G04/src/src/extrator_features.cpp -o CMakeFiles/validacao.dir/src/extrator_features.cpp.s

# Object files for target validacao
validacao_OBJECTS = \
"CMakeFiles/validacao.dir/src/validacao.cpp.o" \
"CMakeFiles/validacao.dir/src/classificador.cpp.o" \
"CMakeFiles/validacao.dir/src/preprocessamento.cpp.o" \
"CMakeFiles/validacao.dir/src/detectorfissura.cpp.o" \
"CMakeFiles/validacao.dir/src/extrator_features.cpp.o"

# External object files for target validacao
validacao_EXTERNAL_OBJECTS =

validacao: CMakeFiles/validacao.dir/src/validacao.cpp.o
validacao: CMakeFiles/validacao.dir/src/classificador.cpp.o
validacao: CMakeFiles/validacao.dir/src/preprocessamento.cpp.o
validacao: CMakeFiles/validacao.dir/src/detectorfissura.cpp.o
validacao: CMakeFiles/validacao.dir/src/extrator_features.cpp.o
validacao: CMakeFiles/validacao.dir/build.make
validacao: /usr/lib/x86_64-linux-gnu/libopencv_stitching.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_alphamat.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_aruco.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_barcode.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_bgsegm.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_bioinspired.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_ccalib.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_cvv.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_dnn_objdetect.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_dnn_superres.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_dpm.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_face.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_freetype.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_fuzzy.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_hdf.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_hfs.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_img_hash.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_intensity_transform.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_line_descriptor.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_mcc.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_quality.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_rapid.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_reg.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_rgbd.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_saliency.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_shape.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_stereo.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_structured_light.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_superres.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_surface_matching.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_tracking.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_videostab.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_viz.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_wechat_qrcode.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_xobjdetect.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_xphoto.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_highgui.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_datasets.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_plot.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_text.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_ml.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_phase_unwrapping.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_optflow.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_ximgproc.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_video.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_videoio.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_imgcodecs.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_objdetect.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_calib3d.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_dnn.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_features2d.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_flann.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_photo.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_imgproc.so.4.6.0
validacao: /usr/lib/x86_64-linux-gnu/libopencv_core.so.4.6.0
validacao: CMakeFiles/validacao.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --bold --progress-dir=/home/inteli/Documentos/modulo6/2025-1B-T12-EC06-G04/src/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_6) "Linking CXX executable validacao"
	$(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/validacao.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
CMakeFiles/validacao.dir/build: validacao
.PHONY : CMakeFiles/validacao.dir/build

CMakeFiles/validacao.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles/validacao.dir/cmake_clean.cmake
.PHONY : CMakeFiles/validacao.dir/clean

CMakeFiles/validacao.dir/depend:
	cd /home/inteli/Documentos/modulo6/2025-1B-T12-EC06-G04/src/build && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /home/inteli/Documentos/modulo6/2025-1B-T12-EC06-G04/src /home/inteli/Documentos/modulo6/2025-1B-T12-EC06-G04/src /home/inteli/Documentos/modulo6/2025-1B-T12-EC06-G04/src/build /home/inteli/Documentos/modulo6/2025-1B-T12-EC06-G04/src/build /home/inteli/Documentos/modulo6/2025-1B-T12-EC06-G04/src/build/CMakeFiles/validacao.dir/DependInfo.cmake "--color=$(COLOR)"
.PHONY : CMakeFiles/validacao.dir/depend

