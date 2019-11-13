tflite now includes the tensorflow git repository as a submodule
along with the results of calling `download_dependencies.sh`.
This reduces the number of build steps from ~260 to less than 100. 
If the version of tensorflow is ever updated, `submodules/update-downloads.sh` 
should also be updated if necessary and called. It removes most of the 
files that are obviously not necessary since they all get committed.
