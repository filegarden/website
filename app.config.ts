const SITE_DOMAIN = "filegarden.com";
const FILE_DOMAIN = "file.garden";

export default defineAppConfig({
  APP_NAME: "File Garden",
  SITE_DOMAIN,
  FILE_DOMAIN,
  SITE_URL_BASE: `https://${SITE_DOMAIN}`,
  FILE_URL_BASE: `https://${FILE_DOMAIN}`,
});
