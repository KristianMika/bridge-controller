import ITool from "./ITool";

interface IToolMenu {
  tools: ITool[];
  addTool: (tool: ITool) => void;
  removeTool: (tool: ITool) => void;
  selectedTool: ITool;
  setSelectedTool: (tool: ITool) => void;
}
export default IToolMenu;
