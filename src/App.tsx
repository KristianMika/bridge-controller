import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";
import { createBrowserRouter, Outlet, RouterProvider } from "react-router-dom";
import { InterfaceConfiguration } from "./components/interfaceConfiguration/InterfaceConfiguration";
import { Menu } from "./components/menu/Menu";
import { MenuSeparator } from "./components/menuSeparator/MenuSeparator";
import { GroupsRequest } from "./mpc";
import { MPCClient } from "./mpc.client";

function App() {
  let meesign_transport = new GrpcWebFetchTransport({
    baseUrl: "http://localhost:1337",
  });
  let meesign_client = new MPCClient(meesign_transport);

  const group_request: GroupsRequest = {};
  meesign_client
    .getGroups(group_request, { "Access-Control-Allow-Origin": "*" })
    .then((response) => {
      alert(response.response);
    });
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
              interfaceType={"Cryptoki"} // TODO
            />
          ),
        },
        {
          path: "cryptoki",
          element: (
            <InterfaceConfiguration
              key="cryptoki"
              canBeDisabled={false}
              interfaceType={"Cryptoki"}
            />
          ),
        },
        {
          path: "webauthn",
          element: (
            <InterfaceConfiguration
              key="webauthn"
              canBeDisabled={true}
              interfaceType={"Webauthn"}
            />
          ),
        },
        {
          path: "pcsc",
          element: (
            <InterfaceConfiguration
              key="pcsc"
              canBeDisabled={true}
              interfaceType={"Pcsc"}
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
