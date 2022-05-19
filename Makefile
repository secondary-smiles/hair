NOW := $(shell date +"%c" | tr ' :' '__')

all:
	@make clean
	@mkdir -p target_build

	@echo "\n\nExpected output $(NOW):" >> target_build/target_build.log
	@echo "[+] Successfully created target_build directory" >> target_build/target_build.log
	@echo "[+] Successfully compiled Hair" >> target_build/target_build.log
	@echo "[+] Successfully added executable permissions" >> target_build/target_build.log

	@echo "target_building Hair.."
	@echo "\n\nBuild output $(NOW):" >> target_build/target_build.log
	@echo "[+] Successfully created target_build directory" >> target_build/target_build.log
	@rustc -o target_build/main hair/src/main.rs
	@echo "[+] Successfully compiled Hair" >> target_build/target_build.log
	@chmod +x target_build/main
	@echo "[+] Successfully added executable permissions" >> target_build/target_build.log
	@echo "[+] Build complete!" >> target_build/target_build.log

	@echo "Please compare target_build/target_build.log output to target_build/target_build.log expected output to verify successful target_build"

clean:
	@echo "Backing up logfiles.."
	@mkdir -p target_build/target_build_bak
	-@cp target_build/target_build.log target_build/target_build_bak/target_build_$(NOW).log.bak
	@echo "Cleaning up.."
	@rm -rf target_build/main
	@rm -rf target_build/target_build.log
	@echo "target_build directory cleaned!"

reset:
	@echo "Resetting target_build directory.."
	@rm -rf target_build
	@echo "target_build directory reset!"

install:
	@echo "\n\n Install Log $(NOW):" >> target_build/target_build.log
	@sudo install target_build/main /usr/local/bin/hair
	@echo "Successfully copied executable to /usr/local/bin"
	@echo "[+] Successfully copied executable to /usr/local/bin" >> target_build/target_build.log

uninstall:
	@echo "\n\n Uninstall Log $(NOW):" >> target_build/target_build.log
	@sudo rm /usr/local/bin/hair
	@echo "Successfully removed executable from /usr/local/bin"
	@echo "[+] Successfully removed executable from /usr/local/bin" >> target_build/target_build.log

update:
	@echo "\n\n Update Log $(NOW):" >> target_build/target_build.log
	@echo "\n\n[+] Updating.." >> target_build/target_build.log
	@echo "Updating hair git repository.."
	@git pull
	@echo "\n\n[+] Rebuilding and installing.." >> target_build/target_build.log
	@make all
	@make install
	@echo "\n\n[+] Successfully updated!" >> target_build/target_build.log
	@echo "Hair git repository updated!"

