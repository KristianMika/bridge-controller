import { createBrowserRouter, Outlet, RouterProvider } from "react-router-dom";
import { InterfaceConfiguration } from "./components/interfaceConfiguration/InterfaceConfiguration";
import { Menu } from "./components/menu/Menu";
import { MenuSeparator } from "./components/menuSeparator/MenuSeparator";

function App() {
  const router = createBrowserRouter([
    {
      path: "/",
      element: (
        <>
          <Menu />
          <MenuSeparator />
          <Outlet />
        </>
      ),
      children: [
        {
          path: "hwi",
          element: (
            <InterfaceConfiguration
              key="hwi"
              canBeDisabled={true}
              interfaceType={"cryptoki"} // TODO
            />
          ),
        },
        {
          path: "cryptoki",
          element: (
            <InterfaceConfiguration
              key="cryptoki"
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
              canBeDisabled={true}
              interfaceType={"webauthn"}
            />
          ),
        },
        {
          path: "pcsc",
          element: (
            <InterfaceConfiguration
              key="pcsc"
              canBeDisabled={true}
              interfaceType={"pcsc"}
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
}

export default App;
