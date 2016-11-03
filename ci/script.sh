mdbook build
cat <<EOF > book/robots.txt
User-agent: *
Disallow: /
EOF
