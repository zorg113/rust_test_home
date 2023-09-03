// State transfer menu items
// AddTask -> AddNameTask -> AddDateTask -> AsSubtask -> SelectTask -> MainMenu
//                                       -> MainMenu
// EndTask -> SelectTask ->MainMenu

struct AddTask {}

struct EndTask {}

struct MainMenu {}

struct SelectTask {}

struct AddNameTask {}

struct AddDateTask {}

struct AsSubTask {}

struct MenuTaskBotMachine<S> {
    state: S,
}

impl MenuTaskBotMachine<MainMenu> {
    fn new() -> Self {
        MenuTaskBotMachine { state: MainMenu {} }
    }
}

impl From<MenuTaskBotMachine<MainMenu>> for MenuTaskBotMachine<AddTask> {
    fn from(val: MenuTaskBotMachine<MainMenu>) -> MenuTaskBotMachine<AddTask> {
        MenuTaskBotMachine { state: AddTask {} }
    }
}

impl From<MenuTaskBotMachine<MainMenu>> for MenuTaskBotMachine<EndTask> {
    fn from(val: MenuTaskBotMachine<MainMenu>) -> MenuTaskBotMachine<EndTask> {
        MenuTaskBotMachine { state: EndTask {} }
    }
}

impl From<MenuTaskBotMachine<EndTask>> for MenuTaskBotMachine<SelectTask> {
    fn from(val: MenuTaskBotMachine<EndTask>) -> MenuTaskBotMachine<SelectTask> {
        MenuTaskBotMachine {
            state: SelectTask {},
        }
    }
}

impl From<MenuTaskBotMachine<AddTask>> for MenuTaskBotMachine<AddNameTask> {
    fn from(val: MenuTaskBotMachine<AddTask>) -> MenuTaskBotMachine<AddNameTask> {
        MenuTaskBotMachine {
            state: AddNameTask {},
        }
    }
}

impl From<MenuTaskBotMachine<AddNameTask>> for MenuTaskBotMachine<AddDateTask> {
    fn from(val: MenuTaskBotMachine<AddNameTask>) -> MenuTaskBotMachine<AddDateTask> {
        MenuTaskBotMachine {
            state: AddDateTask {},
        }
    }
}

impl From<MenuTaskBotMachine<AddDateTask>> for MenuTaskBotMachine<AsSubTask> {
    fn from(val: MenuTaskBotMachine<AddDateTask>) -> MenuTaskBotMachine<AsSubTask> {
        MenuTaskBotMachine {
            state: AsSubTask {},
        }
    }
}

impl From<MenuTaskBotMachine<AsSubTask>> for MenuTaskBotMachine<SelectTask> {
    fn from(val: MenuTaskBotMachine<AsSubTask>) -> MenuTaskBotMachine<SelectTask> {
        MenuTaskBotMachine {
            state: SelectTask {},
        }
    }
}

impl From<MenuTaskBotMachine<SelectTask>> for MenuTaskBotMachine<MainMenu> {
    fn from(val: MenuTaskBotMachine<SelectTask>) -> MenuTaskBotMachine<MainMenu> {
        MenuTaskBotMachine { state: MainMenu {} }
    }
}

impl From<MenuTaskBotMachine<AddDateTask>> for MenuTaskBotMachine<MainMenu> {
    fn from(val: MenuTaskBotMachine<AddDateTask>) -> MenuTaskBotMachine<MainMenu> {
        MenuTaskBotMachine { state: MainMenu {} }
    }
}
