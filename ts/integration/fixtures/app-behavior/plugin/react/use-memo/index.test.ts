import unsnarlPluginReact from "../../../../../../src/plugins/unsnarl-plugin-react/index.js";
import { fixtureSnapshot } from "../../../../../fixture-snapshot.js";

fixtureSnapshot(import.meta.url);
fixtureSnapshot(import.meta.url, {
  plugins: [unsnarlPluginReact],
  slug: "react",
});
