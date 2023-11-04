import { TransitionGroup, CSSTransition } from "react-transition-group";
import IAnimationComponent from "../../../models/IAnimationComponent";
import styles from "./AnimationComponent.module.css";

/**
 * Animates the children component when the `props.uniqueKey` changes.
 */
const AnimationComponent: React.FC<IAnimationComponent> = (props) => {
  return (
    <TransitionGroup className={styles["animation-wrapper"]}>
      <CSSTransition
        key={props.uniqueKey}
        classNames="fade"
        timeout={300}
        className={styles["animation-wrapper"]}
      >
        <div className={styles["animation-wrapper"]}>{props.children}</div>
      </CSSTransition>
    </TransitionGroup>
  );
};

export default AnimationComponent;
