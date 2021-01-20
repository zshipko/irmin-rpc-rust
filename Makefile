USER:=zshipko
BRANCH:=api-overhaul

update:
	curl -O https://raw.githubusercontent.com/${USER}/irmin-rpc/${BRANCH}/src/irmin-rpc/irmin_api.capnp
