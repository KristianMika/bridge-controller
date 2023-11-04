import {
  createBrowserRouter,
  RouterProvider,
  Navigate,
} from "react-router-dom";
import InterfaceConfiguration from "./components/interfaceConfiguration/InterfaceConfiguration";
import MultiToolInterfaceConfiguration from "./components/interfaceConfiguration/MultiToolInterfaceConfiguration";
import Menu from "./components/menu/Menu";
import MenuSeparator from "./components/menuSeparator/MenuSeparator";
import ITool from "./models/ITool";
import AnimatedOutlet from "./components/animation/animatedOutlet/AnimatedOutlet";

const App = () => {
  const anyTool: ITool = { displayName: "All", tool: null };
  const router = createBrowserRouter([
    {
      path: "/",
      element: (
        <>
          <Menu />
          <MenuSeparator />
          <AnimatedOutlet />
        </>
      ),
      children: [
        {
          path: "/",
          element: <Navigate to="/webauthn"></Navigate>,
        },
        {
          path: "cryptoki",
          element: (
            <MultiToolInterfaceConfiguration
              key="cryptoki"
              displayName="Cryptoki"
              canBeDisabled={false}
              interfaceType={"cryptoki"}
            />
          ),
        },
        {
          path: "webauthn",
          element: (
            <InterfaceConfiguration
              key="webauthn"
              displayName="WebAuthn"
              canBeDisabled={true}
              interfaceType={"webauthn"}
              tool={anyTool}
            />
          ),
        },
        {
          path: "pcsc",
          element: (
            <InterfaceConfiguration
              key="pcsc"
              displayName="PC/SC"
              canBeDisabled={true}
              interfaceType={"pcsc"}
              tool={anyTool}
            />
          ),
        },
      ],
    },
  ]);
  return (
    <div className="container">
      <RouterProvider router={router} />
    </div>
  );
};

export default App;
