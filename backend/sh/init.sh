diesel migration generate accounts
dir=`find ./migrations -name "*accounts"`
cat > ${dir}/up.sql << EOF
CREATE TABLE accounts (
  id UUID PRIMARY KEY,
  avatar VARCHAR NOT NULL,
  level VARCHAR NOT NULL,
  preference_id UUID NOT NULL,
  role_ids UUID[] NOT NULL,
  sensitivity_ids UUID[] NOT NULL,
  survey_results_id UUID NOT NULL
)
EOF
diesel migration run
sleep 1

diesel migration generate conduct_codes
dir=`find ./migrations -name "*conduct_codes"`
cat > ${dir}/up.sql << EOF
CREATE TABLE conduct_codes (
  id UUID PRIMARY KEY,
  name VARCHAR NOT NULL,
  description Text[] NOT NULL,
  qualifications Text[] NOT NULL,
  restrictions Text[] NOT NULL,
  examples Text[] NOT NULL,
  sensitivity_ids UUID[] NOT NULL
)
EOF
diesel migration run
sleep 1

diesel migration generate dress_codes
dir=`find ./migrations -name "*dress_codes"`
cat > ${dir}/up.sql << EOF
CREATE TABLE dress_codes (
  id UUID PRIMARY KEY,
  name VARCHAR NOT NULL,
  description Text[] NOT NULL,
  qualifications Text[] NOT NULL,
  restrictions Text[] NOT NULL,
  links Text[] NOT NULL,
  examples Text[] NOT NULL,
  sensitivity_ids UUID[] NOT NULL
)
EOF
diesel migration run
sleep 1

diesel migration generate events
dir=`find ./migrations -name "*events"`
cat > ${dir}/up.sql << EOF
CREATE TABLE events (
  id UUID PRIMARY KEY,
  name VARCHAR NOT NULL,
  description Text[] NOT NULL,
  imgs Text[] NOT NULL,
  links Text[] NOT NULL,
  ticketing Text[] NOT NULL,
  location Text[] NOT NULL,
  directions Text[] NOT NULL,
  map_images Text[] NOT NULL,
  start_time Timestamp,
  end_time Timestamp,
  conduct_code_ids UUID[] NOT NULL,
  dress_code_ids UUID[] NOT NULL,
  other_expectations Text[] NOT NULL,
  account_ids UUID[] NOT NULL,
  diet_ids UUID[] NOT NULL,
  tasting_session_ids UUID[] NOT NULL,
  env_feature_ids UUID[] NOT NULL,
  sensitivity_ids UUID[] NOT NULL
)
EOF
diesel migration run
sleep 1

diesel migration generate logins
dir=`find ./migrations -name "*logins"`
cat > ${dir}/up.sql << EOF
CREATE TABLE logins (
  id UUID PRIMARY KEY,
  email VARCHAR NOT NULL UNIQUE,
  pw_salt VARCHAR NOT NULL,
  pw_hash VARCHAR NOT NULL,
  mfa_salt VARCHAR NOT NULL,
  mfa_hash VARCHAR NOT NULL
)
EOF
diesel migration run
sleep 1

diesel migration generate preferences
dir=`find ./migrations -name "*preferences"`
cat > ${dir}/up.sql << EOF
CREATE TABLE preferences (
  id UUID PRIMARY KEY,
  browser_theme VARCHAR NOT NULL,
  display_name VARCHAR NOT NULL,
  pronouns VARCHAR NOT NULL
)
EOF
diesel migration run
sleep 1

diesel migration generate roles
dir=`find ./migrations -name "*roles"`
cat > ${dir}/up.sql << EOF
CREATE TABLE roles (
  id UUID PRIMARY KEY,
  title VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  responsibility VARCHAR NOT NULL,
  discount VARCHAR NOT NULL,
  seen_by_role Uuid[] NOT NULL
)
EOF
diesel migration run
sleep 1

diesel migration generate sensitivities
dir=`find ./migrations -name "*sensitivities"`
cat > ${dir}/up.sql << EOF
CREATE TABLE sensitivities (
  id UUID PRIMARY KEY,
  name VARCHAR NOT NULL,
  description Text[] NOT NULL,
  links Text[] NOT NULL,
  precautions Text[] NOT NULL,
  imparing BOOL NOT NULL,
  life_threatening BOOL NOT NULL,
  dietary BOOL NOT NULL,
  environmental BOOL NOT NULL,
  social BOOL NOT NULL
)
EOF
diesel migration run
sleep 1

diesel migration generate survey_results
dir=`find ./migrations -name "*survey_results"`
cat > ${dir}/up.sql << EOF
CREATE TABLE survey_results (
  id UUID PRIMARY KEY,
  aesthetics Text[] NOT NULL,
  cognitive Text[] NOT NULL,
  cosmology Text[] NOT NULL,
  environmental Text[] NOT NULL,
  epistemology Text[] NOT NULL,
  ethics Text[] NOT NULL,
  history Text[] NOT NULL,
  isms Text[] NOT NULL,
  law Text[] NOT NULL,
  logic Text[] NOT NULL,
  maths Text[] NOT NULL,
  ontology Text[] NOT NULL,
  political Text[] NOT NULL,
  rhetoric Text[] NOT NULL,
  science Text[] NOT NULL,
  theology Text[] NOT NULL
)
EOF
diesel migration run
sleep 1


diesel migration generate users
dir=`find ./migrations -name "*users"`
cat > ${dir}/up.sql << EOF
CREATE TABLE users (
  id UUID PRIMARY KEY,
  first_name VARCHAR NOT NULL,
  last_name VARCHAR NOT NULL,
  address Text[] NOT NULL,
  address_verified BOOLEAN[] NOT NULL,
  email Text[] NOT NULL UNIQUE,
  email_verified BOOLEAN[] NOT NULL,
  phone Text[] NOT NULL UNIQUE,
  phone_verified BOOLEAN[] NOT NULL,
  taint VARCHAR NOT NULL,
  login_ids UUID[] NOT NULL
)
EOF
diesel migration run
sleep 1