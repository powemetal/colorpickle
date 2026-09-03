import { useMessage } from "./useMessage";

const { afficherMessage } = useMessage();

export default function useColorPicker() {
  async function ouvrirPipette() {
    return "123456"
}
return { ouvrirPipette };
}
