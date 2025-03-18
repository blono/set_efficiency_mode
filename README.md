# set_efficiency_mode

Change the efficiency mode of the process based on the setting values in conf.txt.  
For example, to disable Chrome's efficiency mode and enable Thunderbird's efficiency mode, configure as follows

```
chrome.exe,0
thunderbird.exe,1
```

Created to turn off Chrome's efficiency mode on Windows 11.  
When efficiency mode is enabled, it not only sets `PROCESS_POWER_THROTTLING_EXECUTION_SPEED` but also changes the priority of the process to `IDLE_PRIORITY_CLASS` (because it needs it)  
Note that if you subsequently disable efficiency mode for the same process, the process priority will remain at `IDLE_PRIORITY_CLASS` (because it does not remember the process priority before the change)