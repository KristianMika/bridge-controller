import { useEffect, useState } from "react";
import { Outlet, useLocation } from "react-router-dom";
import AnimationComponent from "../animatedComponent/AnimationComponent";

/**
 * Animates the outlet so that when a route changes, the transition is animated.
 */
const AnimatedOutlet = () => {
  const location = useLocation();
  const [currentPath, setCurrentPath] = useState(location.pathname);

  useEffect(() => {
    if (location.pathname !== currentPath) {
      setCurrentPath(location.pathname);
    }
  }, [location, currentPath]);
  return (
    <AnimationComponent uniqueKey={currentPath}>
      <Outlet />
    </AnimationComponent>
  );
};

export default AnimatedOutlet;
