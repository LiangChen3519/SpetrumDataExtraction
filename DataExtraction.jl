using DataFrames, CSV, Dates
using BenchmarkTools

file_path = "input_data//Tribox_9B8C_Spectra_2025-05-07_00-00-00_to_2025-05-16_09-45-00.dat"

function read_spectrum_file(file_path::String)
    if !isfile(file_path)
        error("File not found: $file_path")
    end
    # first read the contents of the file
    file_content = read(file_path,String);
    blocks = split(file_content, "[Spectrum]");
    blocks = strip.(blocks); # remove leading and trailing whitespace
    # define a dict for storing the data
    # and define several empty arrays for the data
    Spectra = Vector{Dict{String, Any}}() # container for all Dicts
    data_p = r"\[DATA\](?s)(.*?)\[END\] of \[DATA\]"
    #l = length(blocks)
    for block in blocks
        waveLength = Vector{Float64}()
        absorbance = Vector{Float64}()
        id_data = match(r"\n?IDData\s*=\s*(.+?)(?:\n|$)", block)
        datetime_data = match(r"\n?DateTime\s*=\s*(.+?)(?:\n|$)", block)
        temperature_data = match(r"\n?Temperature\s*=\s*(.+?)(?:\n|$)", block)
        data_dict = Dict{String, Any}()
        data_block = match(data_p,  block)
        if isnothing(data_block) || isnothing(id_data) || isnothing(datetime_data) || isnothing(temperature_data)
            #println("Skipping block due to missing key parsts")
            continue
        else
            try
                id_data = strip(id_data.captures[1])
                datetime_data = strip(datetime_data.captures[1])
                temperature_data = strip(temperature_data.captures[1])
                # data blocks belows:
                data_block = strip(data_block.captures[1])
                # let skip the first line
                for line in split(data_block, "\n")[2:end]
                    data_list = split(line, " ")
                    if length(data_list) < 2
                        continue # skip lines that do not have enough data
                    end
                    push!(waveLength, parse(Float64, data_list[2]))
                    push!(absorbance, parse(Float64, data_list[3]))
                end
                # push all data into the data_dict
                data_dict["IDData"] = id_data
                data_dict["DateTime"] = DateTime(datetime_data, "yyyy-mm-dd HH:MM:SS")
                data_dict["temperature"] = temperature_data
                data_dict["waveLength"] = waveLength
                data_dict["absorbance"] = absorbance
                push!(Spectra, data_dict)
            catch e
                println("Error processing block: $e")
            end
        end
    end
    return Spectra
end

# define a function to convert the spectra data into a DataFrame
# also consider some key, like waveLength and absorbance are Vectors and other keys are only one value
function spectra_to_dataframe(spectra::Vector{Dict{String, Any}})::DataFrame
    # get the number from all rows
    all_rows = sum(min(length(s["waveLength"]), length(s["absorbance"])) for s in spectra)
    #println("Total number of rows in the DataFrame: $all_rows")
    # pre-allocate
    iddatas = Vector{String}(undef, all_rows)
    datetimes = Vector{DateTime}(undef, all_rows)
    temperatures = Vector{String}(undef, all_rows)
    wavelengths = Vector{Float64}(undef, all_rows)
    absorbances = Vector{Float64}(undef, all_rows)
    idx = 1
    # injection of data into the pre-allocated vectors
    for record in spectra
        iddata = record["IDData"]
        datetime = record["DateTime"]
        temperature = record["temperature"]
        wavelength = record["waveLength"]
        absorbance = record["absorbance"]
        # ensure that wavelength and absorbance are of the same length
        l = min(length(wavelength), length(absorbance))
        if l == 0
            continue # skip records with no data
        end
        # fill the pre-allocated vectors
         @inbounds @simd for i in 1:l
            iddatas[idx] = iddata
            datetimes[idx] = datetime
            temperatures[idx] = temperature
            wavelengths[idx] = wavelength[i]  # assuming all wavelengths are the same
            absorbances[idx] = absorbance[i]  # assuming all absorbances are the same
            idx += 1
        end
    end
    return DataFrame(
        IDData = iddatas,
        DateTime = datetimes,
        Temperature = temperatures,
        WaveLength = wavelengths,
        Absorbance = absorbances
    )
end

# apply the function to convert the spectra data into a DataFrame
function main(input_file_path::String,output_file_path::String )
    Spectra = read_spectrum_file(input_file_path)
    spectra_df = spectra_to_dataframe(Spectra)
    CSV.write(output_file_path, spectra_df)
end


@btime main(file_path, "output_data/spectra_data.csv")