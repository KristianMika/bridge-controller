import {
  createBrowserRouter,
  Outlet,
  RouterProvider,
  Navigate,
} from "react-router-dom";
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
          path: "/",
          element: <Navigate to="/webauthn"></Navigate>,
        },
        {
          path: "hwi",
          element: (
            <InterfaceConfiguration
              key="hwi"
              displayName="HWI"
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
