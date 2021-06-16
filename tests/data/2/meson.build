# project
project('dprint-plugin-mesonbuild', 'rust')
project('dprint-plugin-mesonbuild', 'rust', 'c', version: '0.1.0')
project('dprint-plugin-mesonbuild',
  ['c', 'cpp'],
  version: '0.1.0',
  meson_version: '>=0.56.0',
  license: 'proprietary',
  default_options: [
    'warning_level=3',
    'default_library=static'])


# project_arguments
add_project_arguments('-fopenmp', language: 'cpp')
add_project_link_arguments('-fopenmp', language: 'cpp')


# if
if meson.get_compiler('cpp').get_id() == 'gcc'
  add_project_arguments('-Wno-unknown-pragmas', language: 'cpp')
elif meson.get_compiler('cpp').get_id() == 'clang'
  add_project_arguments('-ftime-trace', language: 'cpp')
endif


# generate vscode launch.json
run_target('vscode', command: ['.vscode/generate.py'])


# subproject
doctest_dep = subproject('onqtam-doctest').get_variable('doctest_dep')
spdlog_dep = subproject('spdlog', default_options: [
  'external_fmt=true']).get_variable('spdlog_dep')


# dependency
thread_dep = dependency('threads')
llvm_dep = dependency('llvm', version: '>= 4.0', modules: ['amdgpu'])
llvm_dep = dependency('llvm',
  version: '>= 4.0',
  modules: ['amdgpu'],
  optional_modules: ['inteljitevents'])
zlib_dep = dependency('zlib',
  include_type: 'system',
  required: false,
  disabler: true)


# import
fs = import('fs')

# subdir
subdir('core')


# combination
if get_option('build_tests')
  foreach name, info : test_info_list
    test('test_' + name,
      executable('test_' + name,
        info.get('src'),
        include_directories: info.get('inc'),
        link_with: info.get('lnk'),
        dependencies: [doctest_dep, common_deps]),
      args: ['--no-version=true'],
      workdir: meson.project_build_root())
  endforeach
endif


install_headers(
  'common.h',
  'proj/kola.h',
  subdir: 'myproj')

mydep = declare_dependency(
  include_directories: include_directories('.'),
  link_with: mylibrary,
  sources: myheader)
