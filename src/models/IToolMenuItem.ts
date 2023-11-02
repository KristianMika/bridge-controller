import ITool from "./ITool";

interface IToolMenuItem {
  tool: ITool;
  isSelected: boolean;
  setSelected: (tool: ITool) => void;
}

export default IToolMenuItem;
