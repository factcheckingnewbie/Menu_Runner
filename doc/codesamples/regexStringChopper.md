// FILENAME: /workspaces/CTMenu_Runner/ui/main.slint
71                        property <bool> is-toggled: false;
72                        for action in menu_item.actions: Button {
73-                           // text: action;
74-                           text: is-toggled ? "toggled" : action ;
75+                           text: is-toggled ? 
76+                                 (action.match("^([^:]+):([^:]+)") ? action.match("^([^:]+):([^:]+)")[2] : "toggled")
77+                                 : (action.match("^([^:]+):([^:]+)") ? action.match("^([^:]+):([^:]+)")[1] : action);
78                            checkable: true;
79                            property <string> current-action: action;
80                            clicked => {
81                               is-toggled = !is-toggled;
82                                // Explicitly use the action from the loop's current scope
83-                               root.run_command(menu_item.command-template, self.current-action);
84+                               root.run_command(menu_item.command-template, 
85+                                    is-toggled ? 
86+                                    (current-action.match("^([^:]+):([^:]+)") ? current-action.match("^([^:]+):([^:]+)")[2] : current-action)
87+                                    : (current-action.match("^([^:]+):([^:]+)") ? current-action.match("^([^:]+):([^:]+)")[1] : current-action));
88                            }
89                        }

----------------------------------------

   1. Format your button actions like "freeze:unfreeze"
   2. The UI shows "freeze" when not toggled
   3. The UI shows "unfreeze" when toggled
   4. The command sent to run_command is the appropriate part of the string based on toggle state
   5. No external ButtonStateTracker needed - all handled directly in the UI
<<<<<<< HEAD
=======

>>>>>>> 3395cba980a70e2871c978709a26853445f989ec
