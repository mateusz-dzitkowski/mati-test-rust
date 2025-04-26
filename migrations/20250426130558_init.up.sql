create or replace function uuid_generate_v7()
returns uuid as $$
begin
    return encode(
            set_bit(
                    set_bit(
                            overlay(
                                    uuid_send(gen_random_uuid())
                                    placing substring(int8send(floor(extract(epoch from clock_timestamp()) * 1000)::bigint) from 3)
                                    from 1 for 6
                            ),
                            52,
                            1
                    ),
                    53,
                    1
            ),
            'hex'
           )::uuid;
end
$$ language plpgsql volatile;

create or replace function update_updated_at()
returns trigger as $$
begin
    new.updated_at = now();
    return new;
end;
$$ language plpgsql;
