-- Add migration script here
INSERT INTO users (user_id, username, password_hash)
VALUES (
    '312c2ccf-ebe3-456a-b5f2-7576fbe7a228',
    'admin',
    '$argon2id$v=19$m=15000,t=2,p=1$oTH6b2k9brLxKyYhf4rJ1Q$HwU39fNYEQl+8BPryiiv67lg4XMNaJclft5Wgq15pUw'
)