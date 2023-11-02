import ITool from "./ITool";

interface IToolMenuItemCreation {
  isCreationActive: boolean;
  newItemCreationHandler: (tool: ITool) => void;
  cancelCreation: () => void;
}

export default IToolMenuItemCreation;
