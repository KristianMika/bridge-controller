interface IInterfaceForm {
  isEnabled: boolean;
  communicatorUrl: string;
  selectedGroup: string;
}

export const defaultFormData: IInterfaceForm = {
  isEnabled: true,
  communicatorUrl: "",
  selectedGroup: "",
};

export default IInterfaceForm;
