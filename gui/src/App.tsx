import { InterfaceConfiguration } from "./components/interfaceConfiguration/InterfaceConfiguration";
import { Menu } from "./components/menu/Menu";
import { MenuSeparator } from "./components/menuSeparator/MenuSeparator";

function App() {
  return (
    <div className="container">
      <Menu />
      <MenuSeparator />
      <InterfaceConfiguration />
    </div>
  );
}

export default App;
